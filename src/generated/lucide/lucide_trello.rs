use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" x = "3" width = "18" ry = "2" y = "3" rx = "2" ></ rect > < rect width = "3" height = "9" x = "7" y = "7" ></ rect > < rect height = "5" width = "3" x = "14" y = "7" ></ rect > < / > } } pub const LUCIDE_TRELLO : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;