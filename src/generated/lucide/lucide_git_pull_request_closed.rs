use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cx = "6" cy = "6" ></ circle > < path d = "M6 9v12" ></ path > < path d = "m21 3-6 6" ></ path > < path d = "m21 9-6-6" ></ path > < path d = "M18 11.5V15" ></ path > < circle r = "3" cy = "18" cx = "18" ></ circle > < / > } } pub const LUCIDE_GIT_PULL_REQUEST_CLOSED : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;