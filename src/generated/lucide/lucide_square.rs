use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" x = "3" height = "18" width = "18" rx = "2" ></ rect > < / > } } pub const LUCIDE_SQUARE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24")] } ;