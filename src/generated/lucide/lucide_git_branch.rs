use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "15" x2 = "6" x1 = "6" y1 = "3" ></ line > < circle cx = "18" cy = "6" r = "3" ></ circle > < circle cx = "6" r = "3" cy = "18" ></ circle > < path d = "M18 9a9 9 0 0 1-9 9" ></ path > < / > } } pub const LUCIDE_GIT_BRANCH : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;