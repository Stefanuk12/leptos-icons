use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "12" y2 = "22" y1 = "2" x2 = "12" ></ line > < path d = "M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6" ></ path > < / > } } pub const LUCIDE_DOLLAR_SIGN : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;