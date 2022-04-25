# The json_dynamic project

Let's look at the source code of the project:
1. This project gets the pathnames of two files from the command line—the existing JSON file ("input_path") to read into a memory structure and a JSON file to create ("output_path") by saving the loaded structure, after having modified it a bit.
2. Then, the input file is loaded into the string named `sales_and_products_text` and the
generic `serde_json::from_str::<Value>` function is used to parse the string into a dynamically typed structure representing the JSON file. This structure is stored in the `sales_and_products` local variable.

######

Imagine that we want to change the quantity sold by the second sale transaction, incrementing it by 1.5 kilograms:
1. First, we must get to this value using the following expression:
```
  sales_and_products["sales"][1]["quantity"]
```
2. This retrieves the "sales" sub-object of the general object. It is an array containing three objects.
 [ 27 ]
  
### Storing and Retrieving Data
3. Then, this expression gets the second item (starting from zero (`[1]`)) of this array. This is an object representing a single sale transaction.
4. After this, it gets the `"quantity"` sub-object of the sale transaction object.
5. The value we have reached has a dynamic type that we think should be `serde_json::Value::Number`, and so we make a pattern matching with this type, specifying the if let `Value::Number(n)` clause.
6. If all is good, the matching succeeds and we get a variable named n—containing a number, or something that can be converted into a Rust floating-point number by using the `as_f64` function. Lastly, we can increment the Rust number and then create a JSON number from it using the `from_f64` function. We can then assign this object to the JSON structure using the same expression we used to get it:
```      
    sales_and_products["sales"][1]["quantity"]
            = Value::Number(Number::from_f64(n.as_f64().unwrap() + 1.5).unwrap());
```
7. The last statement of the program saves the JSON structure to a file. Here,
the `serde_json::to_string_pretty` function is used. As the name suggests, this function adds formatting whitespace (blanks and new lines) to make the resulting JSON file more human-readable. There is also
the `serde_json::to_string` function, which creates a more compact version of the same information. It is much harder for people to read, but it is somewhat quicker to process for a computer:
```
std::fs::write(
    output_path,
    serde_json::to_string_pretty(&sales_and_products).unwrap(),
).unwrap();
```
