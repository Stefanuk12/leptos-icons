use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" x = "3" rx = "2" y = "3" width = "18" ></ rect > < circle r = "1" cy = "12" cx = "12" ></ circle > < / > } } pub const LUCIDE_SQUARE_DOT : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none")] } ;