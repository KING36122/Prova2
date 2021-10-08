library STD;
USE STANDARD.ALL;


entity circuito is
port(A,B : IN BIT;
     X : OUT BIT);
end circuito;

architecture funcCircuito of circuito is
Begin
    --Negando o NAND
    X <= NOT(NOT(A AND B));
end funcCircuito;
