use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" width = "18" height = "18" y = "3" rx = "2" ></ rect > < circle cy = "12" r = "1" cx = "12" ></ circle > < / > } } pub const LUCIDE_SQUARE_DOT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;