use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20.24 12.24a6 6 0 0 0-8.49-8.49L5 10.5V19h8.5z" ></ path > < line y2 = "22" x2 = "2" y1 = "8" x1 = "16" ></ line > < line y1 = "15" x2 = "9" x1 = "17.5" y2 = "15" ></ line > < / > } } pub const LUCIDE_FEATHER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("fill" , "none")] } ;