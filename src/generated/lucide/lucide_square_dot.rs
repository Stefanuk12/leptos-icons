use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" width = "18" x = "3" y = "3" rx = "2" ></ rect > < circle r = "1" cy = "12" cx = "12" ></ circle > < / > } } pub const LUCIDE_SQUARE_DOT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;