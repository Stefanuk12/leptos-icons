use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" x = "3" y = "3" rx = "1" height = "7" ></ rect > < rect x = "3" y = "14" width = "9" height = "7" rx = "1" ></ rect > < rect width = "5" y = "14" x = "16" rx = "1" height = "7" ></ rect > < / > } } pub const LucideLayoutTemplate : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;