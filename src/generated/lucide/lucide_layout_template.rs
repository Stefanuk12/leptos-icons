use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" y = "3" x = "3" width = "18" height = "7" ></ rect > < rect width = "9" rx = "1" y = "14" height = "7" x = "3" ></ rect > < rect y = "14" rx = "1" height = "7" x = "16" width = "5" ></ rect > < / > } } pub const LucideLayoutTemplate : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;