use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" height = "7" width = "7" x = "3" rx = "1" ></ rect > < rect height = "7" x = "14" y = "3" width = "7" rx = "1" ></ rect > < rect x = "14" rx = "1" height = "7" width = "7" y = "14" ></ rect > < rect x = "3" rx = "1" y = "14" height = "7" width = "7" ></ rect > < / > } } pub const LucideLayoutGrid : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;