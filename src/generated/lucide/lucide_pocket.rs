use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 3h16a2 2 0 0 1 2 2v6a10 10 0 0 1-10 10A10 10 0 0 1 2 11V5a2 2 0 0 1 2-2z" ></ path > < polyline points = "8 10 12 14 16 10" ></ polyline > < / > } } pub const LUCIDE_POCKET : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;