use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" x = "3" rx = "2" ry = "2" width = "18" height = "18" ></ rect > < rect x = "7" width = "3" y = "7" height = "9" ></ rect > < rect width = "3" height = "5" x = "14" y = "7" ></ rect > < / > } } pub const LUCIDE_TRELLO : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;