use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cx = "12" cy = "12" ></ circle > < line x1 = "9" x2 = "15" y1 = "15" y2 = "9" ></ line > < / > } } pub const LUCIDE_CIRCLE_SLASH : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;