use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "15" y1 = "15" x1 = "9" y2 = "9" ></ line > < circle r = "10" cx = "12" cy = "12" ></ circle > < / > } } pub const LUCIDE_CIRCLE_SLASH : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none")] } ;