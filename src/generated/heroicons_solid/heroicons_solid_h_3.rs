use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path clip - rule = "evenodd" fill - rule = "evenodd" d = "M12.749 3.743a.75.75 0 0 1 .75.75v15.002a.75.75 0 1 1-1.5 0v-6.75H2.997v6.75a.75.75 0 0 1-1.5 0V4.494a.75.75 0 1 1 1.5 0v6.75H12v-6.75a.75.75 0 0 1 .75-.75ZM18.75 10.5c-.727 0-1.441.055-2.139.16a.75.75 0 1 1-.223-1.483 15.87 15.87 0 0 1 3.82-.11c.95.088 1.926.705 2.168 1.794a5.265 5.265 0 0 1-.579 3.765 5.265 5.265 0 0 1 .578 3.765c-.24 1.088-1.216 1.706-2.167 1.793a15.942 15.942 0 0 1-3.82-.109.75.75 0 0 1 .223-1.483 14.366 14.366 0 0 0 3.46.099c.467-.043.773-.322.84-.624a3.768 3.768 0 0 0-.413-2.691H18a.75.75 0 0 1 0-1.5h2.498a3.768 3.768 0 0 0 .413-2.69c-.067-.303-.373-.582-.84-.625-.435-.04-.876-.06-1.321-.06Z" ></ path > < / > } } pub const HEROICONS_SOLID_H_3 : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("fill" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("viewBox" , "0 0 24 24")] } ;