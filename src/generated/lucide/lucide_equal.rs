use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "19" x1 = "5" y1 = "9" y2 = "9" ></ line > < line y2 = "15" x1 = "5" x2 = "19" y1 = "15" ></ line > < / > } } pub const LUCIDE_EQUAL : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round")] } ;