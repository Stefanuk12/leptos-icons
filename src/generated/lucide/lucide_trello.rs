use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" rx = "2" width = "18" height = "18" ry = "2" x = "3" ></ rect > < rect y = "7" width = "3" height = "9" x = "7" ></ rect > < rect height = "5" x = "14" width = "3" y = "7" ></ rect > < / > } } pub const LUCIDE_TRELLO : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;