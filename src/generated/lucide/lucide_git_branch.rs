use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "6" y1 = "3" x2 = "6" y2 = "15" ></ line > < circle cx = "18" cy = "6" r = "3" ></ circle > < circle cy = "18" r = "3" cx = "6" ></ circle > < path d = "M18 9a9 9 0 0 1-9 9" ></ path > < / > } } pub const LUCIDE_GIT_BRANCH : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;