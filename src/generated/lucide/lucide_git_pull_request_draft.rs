use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cx = "18" cy = "18" ></ circle > < circle r = "3" cy = "6" cx = "6" ></ circle > < path d = "M18 6V5" ></ path > < path d = "M18 11v-1" ></ path > < line y2 = "21" x1 = "6" y1 = "9" x2 = "6" ></ line > < / > } } pub const LUCIDE_GIT_PULL_REQUEST_DRAFT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24")] } ;