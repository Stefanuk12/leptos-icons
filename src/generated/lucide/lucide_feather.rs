use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20.24 12.24a6 6 0 0 0-8.49-8.49L5 10.5V19h8.5z" ></ path > < line x2 = "2" y1 = "8" x1 = "16" y2 = "22" ></ line > < line y1 = "15" y2 = "15" x1 = "17.5" x2 = "9" ></ line > < / > } } pub const LUCIDE_FEATHER : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;