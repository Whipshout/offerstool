# PC

**CPU**: AMD Ryzen 7 5800X 3.80 GHz<br>
**Motherboard**: ASUS TUF Gaming X570-Plus<br>
**SSD**: AORUS NVMe Gen4 M.2 1TB<br>
**RAM**: Corsair DDR4 32GB 3200 MHZ CL16<br>
**SO**: Windows 10 Pro

<br>

------------------------------------------------------------------------------------------

# Goals
- [x] Show that **Rust is much better than js/ts** and can help us.
- [ ] Start **implementing Rust** in our work pipeline.

<br>


------------------------------------------------------------------------------------------

# RUST

### Perks
1. It is made to be used as a **cli tool**. ./offerstool -h in console to see the application information. You can enter the names of the three files or not put any and use the hardcoded default names.
2. **Remove duplicate** user_ids in files to make the report.
3. In case an error occurs (**something unlikely because it is Rust**), the error is handled by launching a panic and closing the program with an error message. It could have been specified where the error is, but it has not been done due to the use that will be given to the tool.
4. **23** code lines (core).
<br><br>

| Action | Seconds | Milliseconds |
| :---: | :---: | :---: |
| Time reading file mixpanel_dump.csv | 0.0013889s | 1.3889ms |
| Time reading file database_dump.csv | 0.0026363s | 2.6363ms |
| Time formatting file | 0.0106647s | 10.6647ms |
| Time writing file output.csv | 0.001436s | 1.436ms |
| Total time used | 0.0253711s | 25.3711ms |

<br>

![Performance](./comparative/rust_performance.png)

<br>

------------------------------------------------------------------------------------------


# JS

### Perks
1. It is intended to be used by running the code, **not cli tool**.
2. **It does not remove duplicate** user_ids in files to make the report.
3. In case an error occurs (**something very likely because it is Javascript**), the error is not handle and the app just shut down.
4. **27** code lines (core).
<br><br>

| Action | Seconds | Milliseconds |
| :---: | :---: | :---: |
| Time reading file mixpanel_dump.csv | 0.013s | 13ms |
| Time reading file database_dump.csv | 0.025s | 25ms |
| Time formatting file | 0.065s | 65ms |
| Time writing file output.csv | 0.003s | 3ms |
| Total time used | 0.113s | 113ms |

<br>

![Performance](./comparative/js_performance.PNG)

<br>

------------------------------------------------------------------------------------------


# SUMMARY

- Rust reads files about **ten times** faster<br>
- Rust writes files about **three times** faster<br>
- Most of the time is spent **searching** common user_ids and formatting report data<br>
- Rust is **almost six times faster** than js at manipulating data in this case<br>
- **No sorting or search algorithms** have been used, only what the language brings<br>
<br><br>
