use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" y = "3" height = "18" x = "3" width = "18" ></ rect > < rect x = "8" y = "8" width = "8" height = "8" rx = "1" ></ rect > < / > } } pub const LUCIDE_SQUARE_SQUARE : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24")] } ;