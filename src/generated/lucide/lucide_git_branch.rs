use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "15" y1 = "3" x1 = "6" x2 = "6" ></ line > < circle r = "3" cy = "6" cx = "18" ></ circle > < circle cy = "18" r = "3" cx = "6" ></ circle > < path d = "M18 9a9 9 0 0 1-9 9" ></ path > < / > } } pub const LUCIDE_GIT_BRANCH : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24")] } ;