use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" height = "7" y = "3" width = "7" rx = "1" ></ rect > < rect x = "14" width = "7" rx = "1" height = "7" y = "3" ></ rect > < rect x = "14" rx = "1" width = "7" y = "14" height = "7" ></ rect > < rect x = "3" y = "14" height = "7" width = "7" rx = "1" ></ rect > < / > } } pub const LucideLayoutGrid : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;