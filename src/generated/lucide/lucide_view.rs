use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 17v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-2" ></ path > < path d = "M21 7V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v2" ></ path > < circle r = "1" cx = "12" cy = "12" ></ circle > < path d = "M18.944 12.33a1 1 0 0 0 0-.66 7.5 7.5 0 0 0-13.888 0 1 1 0 0 0 0 .66 7.5 7.5 0 0 0 13.888 0" ></ path > < / > } } pub const LUCIDE_VIEW : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("stroke-width" , "2")] } ;