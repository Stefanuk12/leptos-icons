use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" width = "18" x = "3" height = "18" rx = "2" ></ rect > < rect y = "8" width = "8" rx = "1" x = "8" height = "8" ></ rect > < / > } } pub const LUCIDE_SQUARE_SQUARE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;