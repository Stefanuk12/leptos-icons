use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "6" x2 = "6" y1 = "3" y2 = "15" ></ line > < circle cy = "6" cx = "18" r = "3" ></ circle > < circle cx = "6" cy = "18" r = "3" ></ circle > < path d = "M18 9a9 9 0 0 1-9 9" ></ path > < / > } } pub const LUCIDE_GIT_BRANCH : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;