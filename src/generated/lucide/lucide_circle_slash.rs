use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "9" y1 = "15" y2 = "9" x2 = "15" ></ line > < circle cy = "12" r = "10" cx = "12" ></ circle > < / > } } pub const LUCIDE_CIRCLE_SLASH : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2")] } ;