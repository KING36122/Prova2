library STD;
USE STANDARD.ALL;


entity circuito is
port(A,B : IN BIT;
     X : OUT BIT);
end circuito;

architecture funcCircuito of circuito is
Begin
    --Negando a negação de A
    X <= NOT(NOT(A));

    --Negando o NAND
    X <= NOT(NOT(A AND B));

    --Negando o NOR
    X <= NOT(NOT(A OR B));

    --Negando o AND
    X <= NOT(A AND B);

    --Negando o OR
    X <= NOT(A OR B);
end funcCircuito;
