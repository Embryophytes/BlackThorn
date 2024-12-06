[![StandWithUkraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/badges/StandWithUkraine.svg)](https://github.com/vshymanskyy/StandWithUkraine/blob/main/docs/README.md)
[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg)](https://vshymanskyy.github.io/StandWithUkraine/)

# BlackThorn

BlackThorn is an application designed to simplify database management by providing a graphical editor for creating ER diagrams. The tool allows users to define database schemas visually and generate migration files, reducing manual intervention during database updates.

## Overview

With BlackThorn, users can:
- Create tables using a graphical interface.
- Generate SQL code for schema creation (currently supported: **PostgreSQL**).
- Manage database schema changes with minimal manual effort.

While the application is still under active development, its primary goal is to provide a comprehensive solution for database migration management, enabling seamless interaction with target databases.

## Features

- **ER Diagram Editor**:
  - Design database schemas visually by creating and linking tables.
- **Code Generation**:
  - Generate SQL scripts for creating schemas (PostgreSQL only at this stage).
- **Database Interaction**:
  - Future versions will support direct interactions with various database engines.

## Technology Stack

- **Programming Language**: Rust
- **Graphics Engine**: Bevy

## Installation and Usage

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/BlackThorn.git
   cd BlackThorn
   ```

2. Run the application:
   ```bash
   cargo run
   ```

## Future Enhancements

BlackThorn is actively under development, with the following features planned for future releases:

1. **Expanded Database Support**:
   - Support for additional relational databases (e.g., MySQL, SQLite).
   - Consideration for NoSQL databases.

2. **Full Migration Management**:
   - Generate migration files to update existing schemas.
   - Implement version control for migrations.

3. **Storage Options**:
   - Allow saving ER diagrams and generated files for future use.

4. **Direct Database Interaction**:
   - Enable applying migrations directly to target databases.

5. **Enhanced User Interface**:
   - Improve the graphical interface for better usability and user experience.

## Contribution

We welcome contributions from the community! Whether you want to report a bug, suggest a feature, or contribute code, feel free to open an issue or submit a pull request.

## License

BlackThorn is licensed under the [MIT License](LICENSE).

## Acknowledgements

BlackThorn builds upon the **ThornRoot** library for schema manipulation and code generation. Special thanks to the contributors of ThornRoot for their foundational work.

---

Start building your database schemas visually and simplify your database management workflows with **BlackThorn**!
