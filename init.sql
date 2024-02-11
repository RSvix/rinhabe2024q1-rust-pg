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

CREATE PROCEDURE atualizar_saldo(v1 INT, i INT, v2 INT, t CHAR, d VARCHAR(10))
    LANGUAGE SQL
    BEGIN ATOMIC
    UPDATE saldos SET valor = v1 WHERE cliente_id = i;
    INSERT INTO transacoes (cliente_id, valor, tipo, descricao) VALUES (i, v2, t, d);
END;