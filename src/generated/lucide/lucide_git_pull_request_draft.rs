use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "18" r = "3" cy = "18" ></ circle > < circle cx = "6" cy = "6" r = "3" ></ circle > < path d = "M18 6V5" ></ path > < path d = "M18 11v-1" ></ path > < line y2 = "21" x2 = "6" x1 = "6" y1 = "9" ></ line > < / > } } pub const LUCIDE_GIT_PULL_REQUEST_DRAFT : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none")] } ;