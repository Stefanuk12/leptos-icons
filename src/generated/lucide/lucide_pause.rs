use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "4" rx = "1" y = "4" x = "14" height = "16" ></ rect > < rect y = "4" height = "16" rx = "1" x = "6" width = "4" ></ rect > < / > } } pub const LUCIDE_PAUSE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;