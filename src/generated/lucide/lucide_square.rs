use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" height = "18" rx = "2" width = "18" y = "3" ></ rect > < / > } } pub const LUCIDE_SQUARE : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;