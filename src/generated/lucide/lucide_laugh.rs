use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "10" cx = "12" ></ circle > < path d = "M18 13a6 6 0 0 1-6 5 6 6 0 0 1-6-5h12Z" ></ path > < line x1 = "9" y1 = "9" x2 = "9.01" y2 = "9" ></ line > < line x1 = "15" x2 = "15.01" y2 = "9" y1 = "9" ></ line > < / > } } pub const LUCIDE_LAUGH : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;