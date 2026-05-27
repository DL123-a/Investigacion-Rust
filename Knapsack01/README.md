# Knapsack 0/1 en Rust

## Descripción

Este proyecto consiste en una implementación del algoritmo Knapsack 0/1 utilizando programación dinámica en el lenguaje Rust.

El objetivo del algoritmo es maximizar el valor total de los objetos seleccionados sin exceder la capacidad máxima de una mochila. Cada objeto posee un peso y un valor asociado, y únicamente puede seleccionarse completamente o no seleccionarse.

La implementación incluye:
- Programación dinámica mediante tabla bidimensional.
- Reconstrucción de los objetos seleccionados.
- Manejo de casos borde.
- Casos de prueba automáticos.

---

## Requisitos

Para ejecutar el programa es necesario tener instalado Rust.

Puede verificarse la instalación ejecutando el siguiente comando:

```bash
rustc --version
```

---

## Estructura del proyecto

```text
Knapstack01/
│
└── knapsack.rs
```

---

## Compilación y ejecución

1. Descargar el archivo `knapsack.rs`.

2. Abrir una terminal en la carpeta donde se encuentra el archivo.

3. Compilar el programa con el siguiente comando:

```bash
rustc knapsack.rs
```

4. Ejecutar el programa:

### En Linux/macOS

```bash
./main
```

### En Windows

```bash
main.exe
```

---

## Casos de prueba incluidos

La implementación contiene distintos escenarios de prueba para validar el funcionamiento del algoritmo:

1. Caso estándar con múltiples objetos.
2. Mochila con capacidad igual a cero.
3. Objetos cuyo peso excede la capacidad disponible.
4. Lista vacía de objetos.

---

## Funcionamiento del algoritmo

El algoritmo utiliza programación dinámica para construir una tabla donde cada posición representa el valor máximo alcanzable utilizando cierta cantidad de objetos y una capacidad específica.

La relación principal utilizada es:

```text
DP[i][w] = max(
    DP[i-1][w],
    DP[i-1][w-w_i] + v_i
)
```

Donde:
- `w_i` representa el peso del objeto actual.
- `v_i` representa el valor del objeto actual.

---