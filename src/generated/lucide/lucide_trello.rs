use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" y = "3" x = "3" ry = "2" rx = "2" height = "18" ></ rect > < rect width = "3" x = "7" y = "7" height = "9" ></ rect > < rect width = "3" x = "14" height = "5" y = "7" ></ rect > < / > } } pub const LUCIDE_TRELLO : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none")] } ;