use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" height = "7" x = "3" rx = "1" width = "7" ></ rect > < rect width = "7" x = "14" y = "3" rx = "1" height = "7" ></ rect > < rect y = "14" x = "14" width = "7" height = "7" rx = "1" ></ rect > < rect x = "3" rx = "1" width = "7" height = "7" y = "14" ></ rect > < / > } } pub const LucideLayoutGrid : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2")] } ;