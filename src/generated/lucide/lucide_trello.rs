use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" ry = "2" y = "3" height = "18" x = "3" rx = "2" ></ rect > < rect x = "7" width = "3" height = "9" y = "7" ></ rect > < rect y = "7" x = "14" width = "3" height = "5" ></ rect > < / > } } pub const LucideTrello : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;