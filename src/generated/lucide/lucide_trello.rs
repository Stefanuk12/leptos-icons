use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" width = "18" y = "3" height = "18" rx = "2" ry = "2" ></ rect > < rect x = "7" width = "3" height = "9" y = "7" ></ rect > < rect height = "5" x = "14" y = "7" width = "3" ></ rect > < / > } } pub const LUCIDE_TRELLO : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;