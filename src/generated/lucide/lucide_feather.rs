use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20.24 12.24a6 6 0 0 0-8.49-8.49L5 10.5V19h8.5z" ></ path > < line y1 = "8" x2 = "2" x1 = "16" y2 = "22" ></ line > < line y2 = "15" x1 = "17.5" x2 = "9" y1 = "15" ></ line > < / > } } pub const LUCIDE_FEATHER : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor")] } ;