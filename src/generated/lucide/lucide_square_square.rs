use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" height = "18" rx = "2" x = "3" width = "18" ></ rect > < rect width = "8" y = "8" x = "8" rx = "1" height = "8" ></ rect > < / > } } pub const LUCIDE_SQUARE_SQUARE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;