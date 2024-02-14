CREATE TABLE clientes (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(50) UNIQUE NOT NULL,
    limite INTEGER NOT NULL
);

CREATE TABLE transacoes (
    id SERIAL PRIMARY KEY,
    cliente_id INTEGER NOT NULL,
    valor INTEGER NOT NULL,
    tipo CHAR(1) NOT NULL,
    descricao VARCHAR(10) NOT NULL,
    realizada_em TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE saldos (
    id SERIAL PRIMARY KEY,
    cliente_id INTEGER UNIQUE NOT NULL,
    valor INTEGER NOT NULL
);

CREATE INDEX index_cliente_id ON transacoes(cliente_id);
CREATE INDEX index_realizada_em ON transacoes(realizada_em DESC);

---------- INSERTS

DO $$
BEGIN
    INSERT INTO clientes (nome, limite)
    VALUES
        ('o barato sai caro', 1000 * 100),
        ('zan corp ltda', 800 * 100),
        ('les cruders', 10000 * 100),
        ('padaria joia de cocaia', 100000 * 100),
        ('kid mais', 5000 * 100);
    
    INSERT INTO saldos (cliente_id, valor) SELECT id, 0 FROM clientes;
END;
$$;

---------- PROCEDURES

CREATE PROCEDURE reset_db()
    LANGUAGE SQL
    BEGIN ATOMIC
    UPDATE saldos SET valor = 0;
    SELECT 'TRUNCATE TABLE transacoes';
END;

---------- FUNCTIONS

CREATE FUNCTION realizar_transacao(v1 INT, i INT, v2 INT, t CHAR, d VARCHAR(10), l INT, OUT sa INT)
LANGUAGE plpgsql 
AS $$
DECLARE saldo_atual INT;
BEGIN
    SELECT saldos.valor INTO saldo_atual FROM saldos WHERE cliente_id = i FOR UPDATE;
    IF t = 'd' AND (saldo_atual - v2) < (l * -1) THEN
        RAISE EXCEPTION 'saldo insuficiente';
    END IF;
    UPDATE saldos SET valor = valor + v1 WHERE cliente_id = i;
    INSERT INTO transacoes (cliente_id, valor, tipo, descricao) VALUES (i, v2, t, d);
    sa := saldo_atual + v1;
    RETURN;
END;
$$;

CREATE FUNCTION obter_extrato(i INT) RETURNS TABLE (valor INT, tipo CHAR, descricao VARCHAR(10), realizada_em TIMESTAMP, saldo INT)
LANGUAGE plpgsql 
AS $$
DECLARE saldo_atual INT;
BEGIN
    SELECT saldos.valor INTO saldo_atual FROM saldos WHERE cliente_id = i FOR UPDATE;
    RETURN QUERY SELECT transacoes.valor, transacoes.tipo, transacoes.descricao, transacoes.realizada_em, saldo_atual FROM transacoes WHERE cliente_id=$1 ORDER BY realizada_em DESC LIMIT 10;
END;
$$;