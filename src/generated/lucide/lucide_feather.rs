use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20.24 12.24a6 6 0 0 0-8.49-8.49L5 10.5V19h8.5z" ></ path > < line x2 = "2" y2 = "22" y1 = "8" x1 = "16" ></ line > < line x1 = "17.5" y2 = "15" y1 = "15" x2 = "9" ></ line > < / > } } pub const LUCIDE_FEATHER : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;