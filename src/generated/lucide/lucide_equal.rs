use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "5" y2 = "9" x2 = "19" y1 = "9" ></ line > < line x2 = "19" y1 = "15" y2 = "15" x1 = "5" ></ line > < / > } } pub const LUCIDE_EQUAL : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2")] } ;