use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" x = "3" rx = "1" width = "7" height = "7" ></ rect > < rect x = "14" width = "7" rx = "1" height = "7" y = "3" ></ rect > < rect rx = "1" height = "7" x = "14" width = "7" y = "14" ></ rect > < rect width = "7" rx = "1" height = "7" x = "3" y = "14" ></ rect > < / > } } pub const LucideLayoutGrid : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;