use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M13 13v5" ></ path > < path d = "M17 11.47V8" ></ path > < path d = "M17 11h1a3 3 0 0 1 2.745 4.211" ></ path > < path d = "m2 2 20 20" ></ path > < path d = "M5 8v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2v-3" ></ path > < path d = "M7.536 7.535C6.766 7.649 6.154 8 5.5 8a2.5 2.5 0 0 1-1.768-4.268" ></ path > < path d = "M8.727 3.204C9.306 2.767 9.885 2 11 2c1.56 0 2 1.5 3 1.5s1.72-.5 2.5-.5a1 1 0 1 1 0 5c-.78 0-1.5-.5-2.5-.5a3.149 3.149 0 0 0-.842.12" ></ path > < path d = "M9 14.6V18" ></ path > < / > } } pub const LUCIDE_BEER_OFF : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;