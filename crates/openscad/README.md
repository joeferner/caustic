
# Axis Conversion

- Rust x = - OpenSCAD x
- Rust y = OpenSCAD z
- Rust z = OpenSCAD y

# OpenSCAD Features

:white_check_mark: - Completed
:hourglass: - Not Implemented Yet
:x: - No plans for implementation

## Caustic Extensions

- :white_check_mark: `camera(aspect_ratio, image_width, samples_per_pixel, max_depth, vertical_fov, look_from, look_at, defocus_angle, background)`
- :white_check_mark: `lambertian(t)`
- :white_check_mark: `dielectric(n)`
- :white_check_mark: `metal(c, fuzz)`
- :white_check_mark: `checker(scale, even, odd)`
- :white_check_mark: `perlin_turbulence(scale, turbulence_depth)`
- :white_check_mark: `image(filename)`
- :white_check_mark: `quad(q, u, v)`

## Syntax

- :white_check_mark: [`var`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/General#Variables)` = `[`value`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/General#Values_and_Data_Types)`;`
- :white_check_mark: [`var`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/General#Variables)` = cond `[`?`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Conditional_and_Iterator_Functions#Conditional_?_:)` value_if_true `[`:`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Conditional_and_Iterator_Functions#Conditional_?_:)` value_if_false;`
- :hourglass: [`var`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/General#Variables)` = `[`function`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/User-Defined_Functions_and_Modules#Function_Literals)` (x) x + x;`
- :hourglass: [`module`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/User-Defined_Functions_and_Modules#Modules)` name(…) { … }`
- :white_check_mark: [`function`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/User-Defined_Functions_and_Modules#Functions)` name(…) = …`
- :hourglass: [`include`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Include_Statement)` <….scad>`
- :hourglass: [`use`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Include_Statement)` <….scad>`

## Constants

- :white_check_mark: [`undef`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/General#The_Undefined_Value) - undefined value
- :white_check_mark: [`PI`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/General#Numbers) - mathematical constant [π](https://en.wikipedia.org/wiki/Pi) (~3.14159)

## Operators

- :white_check_mark: [`n + m`](https://en.wikibooks.org/w/index.php?title=OpenSCAD_User_Manual/Mathematical_Operators#Scalar_Arithmetical_Operators) - Addition
- :white_check_mark: [`n - m`](https://en.wikibooks.org/w/index.php?title=OpenSCAD_User_Manual/Mathematical_Operators#Scalar_Arithmetical_Operators) - Subtraction
- :white_check_mark: [`n * m`](https://en.wikibooks.org/w/index.php?title=OpenSCAD_User_Manual/Mathematical_Operators#Scalar_Arithmetical_Operators) - Multiplication
- :white_check_mark: [`n / m`](https://en.wikibooks.org/w/index.php?title=OpenSCAD_User_Manual/Mathematical_Operators#Scalar_Arithmetical_Operators) - Division
- :white_check_mark: [`n % m`](https://en.wikibooks.org/w/index.php?title=OpenSCAD_User_Manual/Mathematical_Operators#Scalar_Arithmetical_Operators) - Modulo
- :white_check_mark: [`n ^ m`](https://en.wikibooks.org/w/index.php?title=OpenSCAD_User_Manual/Mathematical_Operators#Scalar_Arithmetical_Operators) - Exponentiation
- :white_check_mark: [`n < m`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Operators#Relational_Operators) - Less Than
- :white_check_mark: [`n <= m`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Operators#Relational_Operators) - Less or Equal
- :white_check_mark: [`b == c`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Operators#Relational_Operators) - Equal
- :white_check_mark: [`b != c`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Operators#Relational_Operators) - Not Equal
- :white_check_mark: [`n >= m`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Operators#Relational_Operators) - Greater or Equal
- :white_check_mark: [`n > m`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Operators#Relational_Operators) - Greater Than
- :white_check_mark: [`b && c`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Operators#Logical_Operators) - Logical And
- :white_check_mark: [`b || c`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Operators#Logical_Operators) - Logical Or
- :white_check_mark: [`!b`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Operators#Logical_Operators) - Negation
- :hourglass: [`b | c`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Operators#Binary_arithmetic) - Binary OR
- :hourglass: [`b & c`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Operators#Binary_arithmetic) - Binary AND
- :hourglass: [`b << c`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Operators#Binary_arithmetic) - Binary Left Shift
- :hourglass: [`b >> c`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Operators#Binary_arithmetic) - Binary Right Shift
- :hourglass: [`~b`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Operators#Binary_arithmetic) - Binary NOT

## Special Variables

- :x: [`$fa`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Other_Language_Features#$fa) - minimum angle
- :x: [`$fs`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Other_Language_Features#$fs) - minimum size
- :x: [`$fn`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Other_Language_Features#$fn) - number of fragments
- :hourglass: [`$t`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Other_Language_Features#$t) - animation step
- :hourglass: [`$vpr`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Other_Language_Features#$vpr) - viewport rotation angles in degrees
- :hourglass: [`$vpt`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Other_Language_Features#$vpt) - viewport translation
- :hourglass: [`$vpd`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Other_Language_Features#$vpd) - viewport camera distance
- :hourglass: [`$vpf`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Other_Language_Features#$vpf) - viewport camera field of view
- :hourglass: [`$children`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/User-Defined_Functions_and_Modules#Children) - number of module children
- :hourglass: [`$preview`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Other_Language_Features#$preview) - true in F5 preview, false for F6

## Modifier Characters

- :hourglass: [`*`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Modifier_Characters#Disable_Modifier) - disable
- :hourglass: [`!`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Modifier_Characters#Root_Modifier) - show only
- :hourglass: [`#`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Modifier_Characters#Debug_Modifier) - highlight / debug
- :hourglass: [`%`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Modifier_Characters#Background_Modifier) - transparent / background

## 2D Primitives

- :hourglass: [`circle`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Using_the_2D_Subsystem#circle)`(radius | d=diameter)`
- :hourglass: [`square`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Using_the_2D_Subsystem#square)`(size, center)`
- :hourglass: [`square`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Using_the_2D_Subsystem#square)`([width,height], center)`
- :hourglass: [`polygon`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Using_the_2D_Subsystem#polygon)`([points])`
- :hourglass: [`polygon`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Using_the_2D_Subsystem#polygon)`([points], [paths])`
- :hourglass: [`text`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Text)`(t, size, font, halign, valign, spacing, direction, language, script)`
- :hourglass: [`import`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Importing_Geometry#import)`("….ext", convexity)` - formats: `DXF|SVG`
- :hourglass: [`projection`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Using_the_2D_Subsystem#3D_to_2D_Projection)`(cut)`

## 3D Primitives

- :white_check_mark: [`sphere`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Primitive_Solids#sphere)`(radius | d=diameter)`
- :hourglass: [`cube`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Primitive_Solids#cube)`(size, center)`
- :white_check_mark: [`cube`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Primitive_Solids#cube)`([width,depth,height], center)`
- :white_check_mark: [`cylinder`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Primitive_Solids#cylinder)`(h, r|d, center)`
- :hourglass: [`cylinder`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Primitive_Solids#cylinder)`(h, r1|d1, r2|d2, center)`
- :hourglass: [`polyhedron`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Primitive_Solids#polyhedron)`(points, faces, convexity)`
- :hourglass: [`import`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Importing_Geometry#import)`("….ext", convexity)` - formats: `STL|OFF|AMF|3MF`
- :hourglass: [`linear_extrude`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Using_the_2D_Subsystem#linear_extrude)`(height, center, convexity, twist, slices)`
- :hourglass: [`rotate_extrude`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Using_the_2D_Subsystem#rotate_extrude)`(angle, convexity)`
- :hourglass: [`surface`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Other_Language_Features#surface)`(file = "….ext", center, convexity)` - formats: `DAT|PNG`

## Transformations

- :white_check_mark: [`translate`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Transformations#translate)`([x,y,z])`
- :white_check_mark: [`rotate`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Transformations#rotate)`([x,y,z])`
- :hourglass: [`rotate`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Transformations#rotate)`(a, [x,y,z])`
- :white_check_mark: [`scale`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Transformations#scale)`([x,y,z])`
- :hourglass: [`resize`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Transformations#resize)`([x,y,z], auto, convexity)`
- :hourglass: [`mirror`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Transformations#mirror)`([x,y,z])`
- :hourglass: [`multmatrix`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Transformations#multmatrix)`(m)`
- :hourglass: [`color`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Transformations#color)`("colorname", alpha)`
- :hourglass: [`color`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Transformations#color)`("#hexvalue") - #rgb|#rgba|#rrggbb|#rrggbbaa`
- :white_check_mark: [`color`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Transformations#color)`([r,g,b,a])`
- :hourglass: [`offset`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Transformations#offset)`(r|delta, chamfer)`
- :hourglass: [`hull`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Transformations#hull)`()`
- :hourglass: [`minkowski`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Transformations#minkowski)`(convexity)`

## Lists

- :white_check_mark: [`list = […, …, …];`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/General#Vectors) - create a list
- :white_check_mark: [`var = list[2];`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/General#Indexing_elements_within_vectors) - index a list (from 0)
- :white_check_mark: [`var = list.z;`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/General#Dot_notation_indexing) - dot notation indexing (x/y/z)

## Boolean Operations

- :hourglass: [`union`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/CSG_Modelling#union)()
- :hourglass: [`difference`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/CSG_Modelling#difference)()
- :hourglass: [`intersection`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/CSG_Modelling#intersection)()

## List Comprehensions

- :hourglass: [`[ for (i = range|list) i ]`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/List_Comprehensions#for) - Generate
- :hourglass: [`[ for (init; condition; next) i ]`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/List_Comprehensions#for) - Generate
- :hourglass: [`[ each i ]`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/List_Comprehensions#each) - Flatten
- :hourglass: [`[ for (i = …) if (condition(i)) i ]`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/List_Comprehensions#if) - Conditions
- :hourglass: [`[ for (i = …) if (condition(i)) x else y ]`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/List_Comprehensions#if/else) - Conditions
- :hourglass: [`[ for (i = …) let (assignments) a ]`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/List_Comprehensions#let) - Assignments

## Flow Control

- :white_check_mark: [`for`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Conditional_and_Iterator_Functions#For_loop)` (i = [start:end]) { … }`
- :hourglass: [`for`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Conditional_and_Iterator_Functions#For_loop)` (i = [start:step:end]) { … }`
- :hourglass: [`for`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Conditional_and_Iterator_Functions#For_loop)` (i = […,…,…]) { … }`
- :hourglass: [`for`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Conditional_and_Iterator_Functions#For_loop)` (i = …, j = …, …) { … }`
- :hourglass: [`intersection_for`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Conditional_and_Iterator_Functions#Intersection_For_Loop)`(i = [start:end]) { … }`
- :hourglass: [`intersection_for`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Conditional_and_Iterator_Functions#Intersection_For_Loop)`(i = [start:step:end]) { … }`
- :hourglass: [`intersection_for`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Conditional_and_Iterator_Functions#Intersection_For_Loop)`(i = […,…,…]) { … }`
- :white_check_mark: [`if`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Conditional_and_Iterator_Functions#If_Statement)` (…) { … }`
- :hourglass: [`let`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Conditional_and_Iterator_Functions#Let_Statement)` (…) { … }`

## Type Test Functions

- :white_check_mark: [`is_undef`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Type_Test_Functions#is_undef)
- :white_check_mark: [`is_bool`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Type_Test_Functions#is_bool)
- :white_check_mark: [`is_num`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Type_Test_Functions#is_num)
- :white_check_mark: [`is_string`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Type_Test_Functions#is_string)
- :white_check_mark: [`is_list`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Type_Test_Functions#is_list)
- :white_check_mark: [`is_function`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Type_Test_Functions#is_function)

## Other

- :white_check_mark: [`echo`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Other_Language_Features#echo)`(…)`
- :hourglass: [`render`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Other_Language_Features#render)`(convexity)`
- :hourglass: [`children`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/User-Defined_Functions_and_Modules#Children)`([idx])`
- :hourglass: [`assert`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Other_Language_Features#assert)`(condition, message)`
- :hourglass: [`assign`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Conditional_and_Iterator_Functions#Assign_Statement) `(…) { … }` (deprecated)

## Functions

- :white_check_mark: [`concat`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#concat)
- :white_check_mark: [`lookup`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#lookup)
- :hourglass: [`str`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/String_Functions#str)
- :hourglass: [`chr`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/String_Functions#chr)
- :hourglass: [`ord`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/String_Functions#ord)
- :hourglass: [`search`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Other_Language_Features#search)
- :hourglass: [`version`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Other_Language_Features#OpenSCAD_Version)
- :hourglass: [`version_num`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Other_Language_Features#OpenSCAD_Version)
- :hourglass: [`parent_module`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Other_Language_Features#parent_module.28n.29_and_.24parent_modules)(idx)

## Mathematical Functions

- :white_check_mark: [`abs`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#abs)
- :white_check_mark: [`sign`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#sign)
- :white_check_mark: [`sin`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#sin)
- :white_check_mark: [`cos`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#cos)
- :white_check_mark: [`tan`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#tan)
- :white_check_mark: [`acos`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#acos)
- :white_check_mark: [`asin`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#asin)
- :white_check_mark: [`atan`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#atan)
- :white_check_mark: [`atan2`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#atan2)
- :white_check_mark: [`floor`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#floor)
- :white_check_mark: [`round`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#round)
- :white_check_mark: [`ceil`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#ceil)
- :white_check_mark: [`ln`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#ln)
- :hourglass: [`len`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#len)
- :hourglass: [`let`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#let)
- :white_check_mark: [`log`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#log)
- :white_check_mark: [`pow`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#pow)
- :white_check_mark: [`sqrt`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#sqrt)
- :white_check_mark: [`exp`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#exp)
- :white_check_mark: [`rands`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#rands)
- :white_check_mark: [`min`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#min)
- :white_check_mark: [`max`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#max)
- :white_check_mark: [`norm`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#norm)
- :white_check_mark: [`cross`](https://en.wikibooks.org/wiki/OpenSCAD_User_Manual/Mathematical_Functions#cross)
