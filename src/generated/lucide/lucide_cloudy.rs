use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M17.5 21H9a7 7 0 1 1 6.71-9h1.79a4.5 4.5 0 1 1 0 9Z" ></ path > < path d = "M22 10a3 3 0 0 0-3-3h-2.207a5.502 5.502 0 0 0-10.702.5" ></ path > < / > } } pub const LUCIDE_CLOUDY : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor")] } ;