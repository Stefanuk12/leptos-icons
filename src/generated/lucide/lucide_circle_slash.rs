use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "9" y2 = "9" x2 = "15" y1 = "15" ></ line > < circle cx = "12" r = "10" cy = "12" ></ circle > < / > } } pub const LUCIDE_CIRCLE_SLASH : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;