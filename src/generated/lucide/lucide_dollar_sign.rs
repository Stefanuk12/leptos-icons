use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "22" x2 = "12" x1 = "12" y1 = "2" ></ line > < path d = "M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6" ></ path > < / > } } pub const LUCIDE_DOLLAR_SIGN : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;