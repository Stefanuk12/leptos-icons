use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" y = "3" ry = "2" x = "3" height = "18" rx = "2" ></ rect > < rect width = "3" x = "7" y = "7" height = "9" ></ rect > < rect x = "14" y = "7" width = "3" height = "5" ></ rect > < / > } } pub const LUCIDE_TRELLO : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;