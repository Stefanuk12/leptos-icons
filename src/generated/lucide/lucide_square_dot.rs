use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" x = "3" y = "3" width = "18" rx = "2" ></ rect > < circle r = "1" cx = "12" cy = "12" ></ circle > < / > } } pub const LUCIDE_SQUARE_DOT : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24")] } ;